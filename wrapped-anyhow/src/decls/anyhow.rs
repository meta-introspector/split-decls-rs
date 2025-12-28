macro_rules! deps {
    () => {
        Result!();
        Error!();
        Ok!();
    };
}

macro_rules! anyhow {
    () => {
        deps!();
        # [doc = " Construct an ad-hoc error from a string or existing non-`anyhow` error"] # [doc = " value."] # [doc = ""] # [doc = " This evaluates to an [`Error`][crate::Error]. It can take either just a"] # [doc = " string, or a format string with arguments. It also can take any custom type"] # [doc = " which implements `Debug` and `Display`."] # [doc = ""] # [doc = " If called with a single argument whose type implements `std::error::Error`"] # [doc = " (in addition to `Debug` and `Display`, which are always required), then that"] # [doc = " Error impl's `source` is preserved as the `source` of the resulting"] # [doc = " `anyhow::Error`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # type V = ();"] # [doc = " #"] # [doc = " use anyhow::{anyhow, Result};"] # [doc = ""] # [doc = " fn lookup(key: &str) -> Result<V> {"] # [doc = "     if key.len() != 16 {"] # [doc = "         return Err(anyhow!(\"key length must be 16 characters, got {:?}\", key));"] # [doc = "     }"] # [doc = ""] # [doc = "     // ..."] # [doc = "     # Ok(())"] # [doc = " }"] # [doc = " ```"] # [macro_export] # [cfg_attr (not (anyhow_no_clippy_format_args) , clippy :: format_args)] macro_rules ! anyhow { ($ msg : literal $ (,) ?) => { $ crate :: __private :: must_use ({ let error = $ crate :: __private :: format_err ($ crate :: __private :: format_args ! ($ msg)) ; error }) } ; ($ err : expr $ (,) ?) => { $ crate :: __private :: must_use ({ use $ crate :: __private :: kind ::*; let error = match $ err { error => (& error) . anyhow_kind () . new (error) , } ; error }) } ; ($ fmt : expr , $ ($ arg : tt) *) => { $ crate :: Error :: msg ($ crate :: __private :: format ! ($ fmt , $ ($ arg) *)) } ; }
    };
}

anyhow!()