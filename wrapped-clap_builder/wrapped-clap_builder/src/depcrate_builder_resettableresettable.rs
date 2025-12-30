// Generated macro for Resettable (enum)
macro_rules! Depcrate_builder_resettableResettable {
() => {
// Module: crate::builder::resettable
// Provides: {"Resettable"}
// Dependencies: {}
# [doc = " Clearable builder value"] # [doc = ""] # [doc = " This allows a builder function to both accept any value that can [`Into::into`] `T` (like"] # [doc = " `&str` into `OsStr`) as well as `None` to reset it to the default.  This is needed to"] # [doc = " workaround a limitation where you can't have a function argument that is `impl Into<Option<T>>`"] # [doc = " where `T` is `impl Into<S>` accept `None` as its type is ambiguous."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::Command;"] # [doc = " # use clap::Arg;"] # [doc = " fn common() -> Command {"] # [doc = "     Command::new(\"cli\")"] # [doc = "         .arg(Arg::new(\"input\").short('i').long(\"input\"))"] # [doc = " }"] # [doc = " let mut command = common();"] # [doc = " command.mut_arg(\"input\", |arg| arg.short(None));"] # [doc = " ```"] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum Resettable < T > { # [doc = " Overwrite builder value"] Value (T) , # [doc = " Reset builder value"] Reset , }
};
}
