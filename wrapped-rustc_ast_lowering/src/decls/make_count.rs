macro_rules! deps {
    () => {
        ArgumentType!();
        LoweringContext!();
    };
}

macro_rules! make_count {
    () => {
        deps!();
        # [doc = " Generate a hir expression for a format_args Count."] # [doc = ""] # [doc = " Generates:"] # [doc = ""] # [doc = " ```text"] # [doc = "     <core::fmt::rt::Count>::Is(…)"] # [doc = " ```"] # [doc = ""] # [doc = " or"] # [doc = ""] # [doc = " ```text"] # [doc = "     <core::fmt::rt::Count>::Param(…)"] # [doc = " ```"] # [doc = ""] # [doc = " or"] # [doc = ""] # [doc = " ```text"] # [doc = "     <core::fmt::rt::Count>::Implied"] # [doc = " ```"] fn make_count < 'hir > (ctx : & mut LoweringContext < '_ , 'hir > , sp : Span , count : & Option < FormatCount > , argmap : & mut FxIndexMap < (usize , ArgumentType) , Option < Span > > ,) -> hir :: Expr < 'hir > { match count { Some (FormatCount :: Literal (n)) => { let count_is = ctx . arena . alloc (ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatCount , sym :: Is ,)) ; let value = ctx . arena . alloc_from_iter ([ctx . expr_u16 (sp , * n)]) ; ctx . expr_call_mut (sp , count_is , value) } Some (FormatCount :: Argument (arg)) => { if let Ok (arg_index) = arg . index { let (i , _) = argmap . insert_full ((arg_index , ArgumentType :: Usize) , arg . span) ; let count_param = ctx . arena . alloc (ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatCount , sym :: Param ,)) ; let value = ctx . arena . alloc_from_iter ([ctx . expr_usize (sp , i)]) ; ctx . expr_call_mut (sp , count_param , value) } else { ctx . expr (sp , hir :: ExprKind :: Err (ctx . dcx () . span_delayed_bug (sp , "lowered bad format_args count") ,) ,) } } None => ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatCount , sym :: Implied) , } }
    };
}

make_count!()