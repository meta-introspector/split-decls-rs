macro_rules! deps {
    () => {
        MacroSubNs!();
    };
}

macro_rules! sub_namespace_match {
    () => {
        deps!();
        # [doc = " Quoted from [rustc]:"] # [doc = " Macro namespace is separated into two sub-namespaces, one for bang macros and"] # [doc = " one for attribute-like macros (attributes, derives)."] # [doc = " We ignore resolutions from one sub-namespace when searching names in scope for another."] # [doc = ""] # [doc = " [rustc]: https://github.com/rust-lang/rust/blob/1.69.0/compiler/rustc_resolve/src/macros.rs#L75"] fn sub_namespace_match (candidate : Option < MacroSubNs > , expected : Option < MacroSubNs >) -> bool { match (candidate , expected) { (Some (candidate) , Some (expected)) => candidate == expected , _ => true , } }
    };
}

sub_namespace_match!()