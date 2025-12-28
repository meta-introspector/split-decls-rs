macro_rules! deps {
    () => {
        Expr!();
        Array!();
        InlineAsm!();
        OffsetOf!();
        Const!();
        Literal!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl Expr { pub fn precedence (& self) -> ast :: prec :: ExprPrecedence { use ast :: prec :: ExprPrecedence ; match self { Expr :: Array (_) | Expr :: InlineAsm (_) | Expr :: Block { .. } | Expr :: Unsafe { .. } | Expr :: Const (_) | Expr :: Async { .. } | Expr :: If { .. } | Expr :: Literal (_) | Expr :: Loop { .. } | Expr :: Match { .. } | Expr :: Missing | Expr :: Path (_) | Expr :: RecordLit { .. } | Expr :: Tuple { .. } | Expr :: OffsetOf (_) | Expr :: Underscore => ExprPrecedence :: Unambiguous , Expr :: Await { .. } | Expr :: Call { .. } | Expr :: Field { .. } | Expr :: Index { .. } | Expr :: MethodCall { .. } => ExprPrecedence :: Postfix , Expr :: Box { .. } | Expr :: Let { .. } | Expr :: UnaryOp { .. } | Expr :: Ref { .. } => { ExprPrecedence :: Prefix } Expr :: Cast { .. } => ExprPrecedence :: Cast , Expr :: BinaryOp { op , .. } => match op { None => ExprPrecedence :: Unambiguous , Some (BinaryOp :: LogicOp (LogicOp :: Or)) => ExprPrecedence :: LOr , Some (BinaryOp :: LogicOp (LogicOp :: And)) => ExprPrecedence :: LAnd , Some (BinaryOp :: CmpOp (_)) => ExprPrecedence :: Compare , Some (BinaryOp :: Assignment { .. }) => ExprPrecedence :: Assign , Some (BinaryOp :: ArithOp (arith_op)) => match arith_op { ArithOp :: Add | ArithOp :: Sub => ExprPrecedence :: Sum , ArithOp :: Mul | ArithOp :: Div | ArithOp :: Rem => ExprPrecedence :: Product , ArithOp :: Shl | ArithOp :: Shr => ExprPrecedence :: Shift , ArithOp :: BitXor => ExprPrecedence :: BitXor , ArithOp :: BitOr => ExprPrecedence :: BitOr , ArithOp :: BitAnd => ExprPrecedence :: BitAnd , } , } , Expr :: Assignment { .. } => ExprPrecedence :: Assign , Expr :: Become { .. } | Expr :: Break { .. } | Expr :: Closure { .. } | Expr :: Return { .. } | Expr :: Yeet { .. } | Expr :: Yield { .. } => ExprPrecedence :: Jump , Expr :: Continue { .. } => ExprPrecedence :: Unambiguous , Expr :: Range { .. } => ExprPrecedence :: Range , } } }
    };
}

impl_227!()