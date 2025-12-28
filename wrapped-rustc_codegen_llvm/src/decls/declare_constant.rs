macro_rules! declare_constant {
    () => {
        # [doc = " Helper macro to let us redeclare gimli's constants as our own constants"] # [doc = " with a different type, with less risk of copy-paste errors."] macro_rules ! declare_constant { ($ name : ident : $ type : ty) => { # [allow (non_upper_case_globals)] pub (crate) const $ name : $ type = :: gimli :: constants ::$ name . 0 as $ type ; const _ : () = assert ! ($ name as i128 == :: gimli :: constants ::$ name . 0 as i128) ; } ; }
    };
}

declare_constant!();