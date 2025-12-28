macro_rules! deps {
    () => {
        ClosureKind!();
        InlineAsm!();
        CaptureBy!();
        ExprId!();
        PatId!();
        LabelId!();
        Const!();
        Literal!();
        Statement!();
        OffsetOf!();
        RecordLitField!();
        Array!();
        MatchArm!();
    };
}

macro_rules! Expr {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub enum Expr { # [doc = " This is produced if the syntax tree does not have a required expression piece."] Missing , Path (Path) , If { condition : ExprId , then_branch : ExprId , else_branch : Option < ExprId > , } , Let { pat : PatId , expr : ExprId , } , Block { id : Option < BlockId > , statements : Box < [Statement] > , tail : Option < ExprId > , label : Option < LabelId > , } , Async { id : Option < BlockId > , statements : Box < [Statement] > , tail : Option < ExprId > , } , Const (ExprId) , Unsafe { id : Option < BlockId > , statements : Box < [Statement] > , tail : Option < ExprId > , } , Loop { body : ExprId , label : Option < LabelId > , } , Call { callee : ExprId , args : Box < [ExprId] > , } , MethodCall { receiver : ExprId , method_name : Name , args : Box < [ExprId] > , generic_args : Option < Box < GenericArgs > > , } , Match { expr : ExprId , arms : Box < [MatchArm] > , } , Continue { label : Option < LabelId > , } , Break { expr : Option < ExprId > , label : Option < LabelId > , } , Return { expr : Option < ExprId > , } , Become { expr : ExprId , } , Yield { expr : Option < ExprId > , } , Yeet { expr : Option < ExprId > , } , RecordLit { path : Option < Box < Path > > , fields : Box < [RecordLitField] > , spread : Option < ExprId > , } , Field { expr : ExprId , name : Name , } , Await { expr : ExprId , } , Cast { expr : ExprId , type_ref : TypeRefId , } , Ref { expr : ExprId , rawness : Rawness , mutability : Mutability , } , Box { expr : ExprId , } , UnaryOp { expr : ExprId , op : UnaryOp , } , # [doc = " `op` cannot be bare `=` (but can be `op=`), these are lowered to `Assignment` instead."] BinaryOp { lhs : ExprId , rhs : ExprId , op : Option < BinaryOp > , } , Assignment { target : PatId , value : ExprId , } , Range { lhs : Option < ExprId > , rhs : Option < ExprId > , range_type : RangeOp , } , Index { base : ExprId , index : ExprId , } , Closure { args : Box < [PatId] > , arg_types : Box < [Option < TypeRefId >] > , ret_type : Option < TypeRefId > , body : ExprId , closure_kind : ClosureKind , capture_by : CaptureBy , } , Tuple { exprs : Box < [ExprId] > , } , Array (Array) , Literal (Literal) , Underscore , OffsetOf (OffsetOf) , InlineAsm (InlineAsm) , }
    };
}

Expr!()