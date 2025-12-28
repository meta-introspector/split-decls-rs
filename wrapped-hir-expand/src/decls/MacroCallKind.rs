macro_rules! deps {
    () => {
        ExpandTo!();
        MacroCallId!();
        EagerCallInfo!();
    };
}

macro_rules! MacroCallKind {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum MacroCallKind { FnLike { ast_id : AstId < ast :: MacroCall > , expand_to : ExpandTo , # [doc = " Some if this is a macro call for an eager macro. Note that this is `None`"] # [doc = " for the eager input macro file."] eager : Option < Arc < EagerCallInfo > > , } , Derive { ast_id : AstId < ast :: Adt > , # [doc = " Syntactical index of the invoking `#[derive]` attribute."] # [doc = ""] # [doc = " Outer attributes are counted first, then inner attributes. This does not support"] # [doc = " out-of-line modules, which may have attributes spread across 2 files!"] derive_attr_index : AttrId , # [doc = " Index of the derive macro in the derive attribute"] derive_index : u32 , # [doc = " The \"parent\" macro call."] # [doc = " We will resolve the same token tree for all derive macros in the same derive attribute."] derive_macro_id : MacroCallId , } , Attr { ast_id : AstId < ast :: Item > , attr_args : Option < Arc < tt :: TopSubtree > > , # [doc = " Syntactical index of the invoking `#[attribute]`."] # [doc = ""] # [doc = " Outer attributes are counted first, then inner attributes. This does not support"] # [doc = " out-of-line modules, which may have attributes spread across 2 files!"] invoc_attr_index : AttrId , } , }
    };
}

MacroCallKind!()