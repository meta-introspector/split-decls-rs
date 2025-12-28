macro_rules! deps {
    () => {
        ExprId!();
        Expr!();
        BindingId!();
        PatId!();
        RecordFieldPat!();
    };
}

macro_rules! Pat {
    () => {
        deps!();
        # [doc = " Close relative to rustc's hir::PatKind"] # [derive (Debug , Clone , Eq , PartialEq)] pub enum Pat { Missing , Wild , Tuple { args : Box < [PatId] > , ellipsis : Option < u32 > , } , Or (Box < [PatId] >) , Record { path : Option < Box < Path > > , args : Box < [RecordFieldPat] > , ellipsis : bool , } , Range { start : Option < ExprId > , end : Option < ExprId > , range_type : RangeOp , } , Slice { prefix : Box < [PatId] > , slice : Option < PatId > , suffix : Box < [PatId] > , } , # [doc = " This might refer to a variable if a single segment path (specifically, on destructuring assignment)."] Path (Path) , Lit (ExprId) , Bind { id : BindingId , subpat : Option < PatId > , } , TupleStruct { path : Option < Box < Path > > , args : Box < [PatId] > , ellipsis : Option < u32 > , } , Ref { pat : PatId , mutability : Mutability , } , Box { inner : PatId , } , ConstBlock (ExprId) , # [doc = " An expression inside a pattern. That can only occur inside assignments."] # [doc = ""] # [doc = " E.g. in `(a, *b) = (1, &mut 2)`, `*b` is an expression."] Expr (ExprId) , }
    };
}

Pat!()