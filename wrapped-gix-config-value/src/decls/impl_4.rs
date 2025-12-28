macro_rules! deps {
    () => {
        Error!();
        Boolean!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [doc = " # Warning"] # [doc = ""] # [doc = " The direct usage of `try_from(\"string\")` is discouraged as it will produce the wrong result for values"] # [doc = " obtained from `core.bool-implicit-true`, which have no separator and are implicitly true."] # [doc = " This method chooses to work correctly for `core.bool-empty=`, which is an empty string and resolves"] # [doc = " to being `false`."] # [doc = ""] # [doc = " Instead of this, obtain booleans with `config.boolean(…)`, which handles the case were no separator is"] # [doc = " present correctly."] impl TryFrom < & BStr > for Boolean { type Error = Error ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { if parse_true (value) { Ok (Boolean (true)) } else if parse_false (value) { Ok (Boolean (false)) } else { use std :: str :: FromStr ; if let Some (integer) = value . to_str () . ok () . and_then (| s | i64 :: from_str (s) . ok ()) { Ok (Boolean (integer != 0)) } else { Err (bool_err (value)) } } } }
    };
}

impl_4!()