macro_rules! deps {
    () => {
        PathParser!();
        Stage!();
        SharedContext!();
    };
}

macro_rules! FinalizeContext {
    () => {
        deps!();
        # [doc = " Context given to every attribute parser during finalization."] # [doc = ""] # [doc = " Gives [`AttributeParser`](crate::attributes::AttributeParser)s enough information to create"] # [doc = " errors, for example."] pub (crate) struct FinalizeContext < 'p , 'sess , S : Stage > { pub (crate) shared : SharedContext < 'p , 'sess , S > , # [doc = " A list of all attribute on this syntax node."] # [doc = ""] # [doc = " Useful for compatibility checks with other attributes in [`finalize`](crate::attributes::AttributeParser::finalize)"] # [doc = ""] # [doc = " Usually, you should use normal attribute parsing logic instead,"] # [doc = " especially when making a *denylist* of other attributes."] pub (crate) all_attrs : & 'p [PathParser < 'p >] , }
    };
}

FinalizeContext!();