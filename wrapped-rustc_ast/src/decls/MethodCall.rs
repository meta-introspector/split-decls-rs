macro_rules! deps {
    () => {
        PathSegment!();
        Expr!();
    };
}

macro_rules! MethodCall {
    () => {
        deps!();
        # [doc = " A method call (e.g. `x.foo::<Bar, Baz>(a, b, c)`)."] # [derive (Clone , Encodable , Decodable , Debug)] pub struct MethodCall { # [doc = " The method name and its generic arguments, e.g. `foo::<Bar, Baz>`."] pub seg : PathSegment , # [doc = " The receiver, e.g. `x`."] pub receiver : Box < Expr > , # [doc = " The arguments, e.g. `a, b, c`."] pub args : ThinVec < Box < Expr > > , # [doc = " The span of the function, without the dot and receiver e.g. `foo::<Bar,"] # [doc = " Baz>(a, b, c)`."] pub span : Span , }
    };
}

MethodCall!();