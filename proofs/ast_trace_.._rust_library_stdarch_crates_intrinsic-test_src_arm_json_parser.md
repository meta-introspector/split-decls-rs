# AST Trace: ../rust/library/stdarch/crates/intrinsic-test/src/arm/json_parser.rs

Generated 8 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use super::intrinsic::ArmIntrinsicType;
use crate::common::argument::{Argument, ArgumentList};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=STRUCT | NAME=ReturnType | COMPLEXITY=2 | LINES=13

```rust
use crate::common::constraint::Constraint;
use crate::common::intrinsic::Intrinsic;
use crate::common::intrinsic_helpers::IntrinsicType;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct ReturnType {
    value: String,
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=17

```rust
#[derive(Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum ArgPrep {
    Register {
        #[serde(rename = "register")]
        #[allow(dead_code)]
        reg: String,
    },
    Immediate {
        #[serde(rename = "minimum")]
        min: i64,
        #[serde(rename = "maximum")]
        max: i64,
    },
    Nothing {},
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=try_from | COMPLEXITY=5 | LINES=8

```rust
impl TryFrom<Value> for ArgPrep {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=JsonIntrinsic | COMPLEXITY=2 | LINES=13

```rust
#[derive(Deserialize, Debug)]
struct JsonIntrinsic {
    #[serde(rename = "SIMD_ISA")]
    simd_isa: String,
    name: String,
    arguments: Vec<String>,
    return_type: ReturnType,
    #[serde(rename = "Arguments_Preparation")]
    args_prep: Option<HashMap<String, Value>>,
    #[serde(rename = "Architectures")]
    architectures: Vec<String>,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=get_neon_intrinsics | COMPLEXITY=8 | LINES=21

```rust
pub fn get_neon_intrinsics(
    filename: &Path,
    target: &str,
) -> Result<Vec<Intrinsic<ArmIntrinsicType>>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let json: Vec<JsonIntrinsic> = serde_json::from_reader(reader).expect("Couldn't parse JSON");

    let parsed = json
        .into_iter()
        .filter_map(|intr| {
            if intr.simd_isa == "Neon" {
                Some(json_to_intrinsic(intr, target).expect("Couldn't parse JSON"))
            } else {
                None
            }
        })
        .collect();
    Ok(parsed)
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=json_to_intrinsic | COMPLEXITY=13 | LINES=45

```rust
fn json_to_intrinsic(
    mut intr: JsonIntrinsic,
    target: &str,
) -> Result<Intrinsic<ArmIntrinsicType>, Box<dyn std::error::Error>> {
    let name = intr.name.replace(['[', ']'], "");

    let results = ArmIntrinsicType::from_c(&intr.return_type.value, target)?;

    let args = intr
        .arguments
        .into_iter()
        .enumerate()
        .map(|(i, arg)| {
            let (type_name, arg_name) = Argument::<ArmIntrinsicType>::type_and_name_from_c(&arg);
            let metadata = intr.args_prep.as_mut();
            let metadata = metadata.and_then(|a| a.remove(arg_name));
            let arg_prep: Option<ArgPrep> = metadata.and_then(|a| a.try_into().ok());
            let constraint: Option<Constraint> = arg_prep.and_then(|a| a.try_into().ok());
            let ty = ArmIntrinsicType::from_c(type_name, target)
                .unwrap_or_else(|_| panic!("Failed to parse argument '{arg}'"));

            let mut arg =
                Argument::<ArmIntrinsicType>::new(i, String::from(arg_name), ty, constraint);

            // The JSON doesn't list immediates as const
            let IntrinsicType {
                ref mut constant, ..
            } = arg.ty.data;
            if arg.name.starts_with("imm") {
                *constant = true
            }
            arg
        })
        .collect();

    let arguments = ArgumentList::<ArmIntrinsicType> { args };

    Ok(Intrinsic {
        name,
        arguments,
        results: results,
        arch_tags: intr.architectures,
    })
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=try_from | COMPLEXITY=19 | LINES=21

```rust
/// ARM-specific
impl TryFrom<ArgPrep> for Constraint {
    type Error = ();

    fn try_from(prep: ArgPrep) -> Result<Self, Self::Error> {
        let parsed_ints = match prep {
            ArgPrep::Immediate { min, max } => Ok((min, max)),
            _ => Err(()),
        };
        if let Ok((min, max)) = parsed_ints {
            if min == max {
                Ok(Constraint::Equal(min))
            } else {
                Ok(Constraint::Range(min..max + 1))
            }
        } else {
            Err(())
        }
    }
}
```

---
*Generated by AST tracing system*
