macro_rules! deps {
    () => {
        Repetition!();
        ClassUnicode!();
        Range!();
        Error!();
        Result!();
        ClassSetItem!();
        Concat!();
        Ast!();
        ClassPerl!();
        Literal!();
        ClassSetBinaryOp!();
        Alternation!();
        Visitor!();
        Writer!();
        Group!();
        Dot!();
        ClassBracketed!();
        Assertion!();
        Flags!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < W : fmt :: Write > Visitor for Writer < W > { type Output = () ; type Err = fmt :: Error ; fn finish (self) -> fmt :: Result { Ok (()) } fn visit_pre (& mut self , ast : & Ast) -> fmt :: Result { match * ast { Ast :: Group (ref x) => self . fmt_group_pre (x) , Ast :: ClassBracketed (ref x) => self . fmt_class_bracketed_pre (x) , _ => Ok (()) , } } fn visit_post (& mut self , ast : & Ast) -> fmt :: Result { match * ast { Ast :: Empty (_) => Ok (()) , Ast :: Flags (ref x) => self . fmt_set_flags (x) , Ast :: Literal (ref x) => self . fmt_literal (x) , Ast :: Dot (_) => self . wtr . write_str (".") , Ast :: Assertion (ref x) => self . fmt_assertion (x) , Ast :: ClassPerl (ref x) => self . fmt_class_perl (x) , Ast :: ClassUnicode (ref x) => self . fmt_class_unicode (x) , Ast :: ClassBracketed (ref x) => self . fmt_class_bracketed_post (x) , Ast :: Repetition (ref x) => self . fmt_repetition (x) , Ast :: Group (ref x) => self . fmt_group_post (x) , Ast :: Alternation (_) => Ok (()) , Ast :: Concat (_) => Ok (()) , } } fn visit_alternation_in (& mut self) -> fmt :: Result { self . wtr . write_str ("|") } fn visit_class_set_item_pre (& mut self , ast : & ast :: ClassSetItem ,) -> Result < () , Self :: Err > { match * ast { ast :: ClassSetItem :: Bracketed (ref x) => { self . fmt_class_bracketed_pre (x) } _ => Ok (()) , } } fn visit_class_set_item_post (& mut self , ast : & ast :: ClassSetItem ,) -> Result < () , Self :: Err > { use crate :: ast :: ClassSetItem :: * ; match * ast { Empty (_) => Ok (()) , Literal (ref x) => self . fmt_literal (x) , Range (ref x) => { self . fmt_literal (& x . start) ? ; self . wtr . write_str ("-") ? ; self . fmt_literal (& x . end) ? ; Ok (()) } Ascii (ref x) => self . fmt_class_ascii (x) , Unicode (ref x) => self . fmt_class_unicode (x) , Perl (ref x) => self . fmt_class_perl (x) , Bracketed (ref x) => self . fmt_class_bracketed_post (x) , Union (_) => Ok (()) , } } fn visit_class_set_binary_op_in (& mut self , ast : & ast :: ClassSetBinaryOp ,) -> Result < () , Self :: Err > { self . fmt_class_set_binary_op_kind (& ast . kind) } }
    };
}

impl_29!();