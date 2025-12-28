macro_rules! ItemKind {
    () => {
        # [derive (Copy , Clone)] enum ItemKind { NakedAsm , InlineAsm , NonAsm , Err , }
    };
}

ItemKind!();