macro_rules! deps {
    () => {
        CrateInfo!();
    };
}

macro_rules! ignored_for_lto {
    () => {
        deps!();
        # [doc = " Returns a boolean indicating whether the specified crate should be ignored"] # [doc = " during LTO."] # [doc = ""] # [doc = " Crates ignored during LTO are not lumped together in the \"massive object"] # [doc = " file\" that we create and are linked in their normal rlib states. See"] # [doc = " comments below for what crates do not participate in LTO."] # [doc = ""] # [doc = " It's unusual for a crate to not participate in LTO. Typically only"] # [doc = " compiler-specific and unstable crates have a reason to not participate in"] # [doc = " LTO."] pub fn ignored_for_lto (sess : & Session , info : & CrateInfo , cnum : CrateNum) -> bool { ! sess . target . no_builtins && (info . compiler_builtins == Some (cnum) || info . is_no_builtins . contains (& cnum)) }
    };
}

ignored_for_lto!();