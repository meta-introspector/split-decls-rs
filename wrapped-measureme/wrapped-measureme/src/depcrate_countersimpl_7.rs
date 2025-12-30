// Generated macro for impl_7 (impl)
macro_rules! Depcrate_countersimpl_7 {
() => {
// Module: crate::counters
// Provides: {"impl_7"}
// Dependencies: {}
impl Counter { pub fn by_name (name : & str) -> Result < Self , Box < dyn Error + Send + Sync > > { Ok (match name { WallTime :: NAME => Counter :: WallTime (WallTime :: new ()) , Instructions :: NAME => Counter :: Instructions (Instructions :: new () ?) , InstructionsMinusIrqs :: NAME => { Counter :: InstructionsMinusIrqs (InstructionsMinusIrqs :: new () ?) } InstructionsMinusRaw0420 :: NAME => { Counter :: InstructionsMinusRaw0420 (InstructionsMinusRaw0420 :: new () ?) } _ => return Err (format ! ("{:?} is not a valid counter name" , name) . into ()) , }) } pub (super) fn describe_as_json (& self) -> String { let (name , units) = match self { Counter :: WallTime (_) => (WallTime :: NAME , r#"[["ns", 1], ["μs", 1000], ["ms", 1000000], ["s", 1000000000]]"# ,) , Counter :: Instructions (_) => (Instructions :: NAME , r#"[["instructions", 1]]"#) , Counter :: InstructionsMinusIrqs (_) => { (InstructionsMinusIrqs :: NAME , r#"[["instructions", 1]]"#) } Counter :: InstructionsMinusRaw0420 (_) => { (InstructionsMinusRaw0420 :: NAME , r#"[["instructions", 1]]"#) } } ; format ! (r#"{{ "name": "{}", "units": {} }}"# , name , units) } # [inline] pub (super) fn since_start (& self) -> u64 { match self { Counter :: WallTime (counter) => counter . since_start () , Counter :: Instructions (counter) => counter . since_start () , Counter :: InstructionsMinusIrqs (counter) => counter . since_start () , Counter :: InstructionsMinusRaw0420 (counter) => counter . since_start () , } } }
};
}
