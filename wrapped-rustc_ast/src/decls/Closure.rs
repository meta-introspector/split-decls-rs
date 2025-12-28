macro_rules! deps {
    () => {
        Walkable!();
        CaptureBy!();
        Expr!();
        Const!();
        ClosureBinder!();
        FnDecl!();
        CoroutineKind!();
    };
}

macro_rules! Closure {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Closure { pub binder : ClosureBinder , pub capture_clause : CaptureBy , pub constness : Const , pub coroutine_kind : Option < CoroutineKind > , pub movability : Movability , pub fn_decl : Box < FnDecl > , pub body : Box < Expr > , # [doc = " The span of the declaration block: 'move |...| -> ...'"] pub fn_decl_span : Span , # [doc = " The span of the argument block `|...|`"] pub fn_arg_span : Span , }
    };
}

Closure!();