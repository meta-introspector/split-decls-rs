// Generated macro for miette_adapter (module)
macro_rules! Depcrate_errormiette_adapter {
() => {
// Module: crate::error
// Provides: {"miette_adapter"}
// Dependencies: {}
# [cfg (feature = "miette-error")] mod miette_adapter { use alloc :: string :: ToString ; use core :: fmt ; use std :: boxed :: Box ; use crate :: error :: LineColLocation ; use super :: { Error , RuleType } ; use miette :: { Diagnostic , LabeledSpan , SourceCode } ; # [derive (Debug)] pub (crate) struct MietteAdapter < R : RuleType > (pub (crate) Error < R >) ; impl < R : RuleType > Diagnostic for MietteAdapter < R > { fn source_code (& self) -> Option < & dyn SourceCode > { Some (& self . 0 . line) } fn labels (& self) -> Option < Box < dyn Iterator < Item = LabeledSpan > > > { let message = self . 0 . variant . message () . to_string () ; let (offset , length) = match self . 0 . line_col { LineColLocation :: Pos ((_ , c)) => (c - 1 , 1) , LineColLocation :: Span ((_ , start_c) , (_ , end_c)) => { (start_c - 1 , end_c - start_c + 1) } } ; let span = LabeledSpan :: new (Some (message) , offset , length) ; Some (Box :: new (std :: iter :: once (span))) } fn help < 'a > (& 'a self) -> Option < Box < dyn fmt :: Display + 'a > > { Some (Box :: new (self . 0 . message ())) } } impl < R : RuleType > fmt :: Display for MietteAdapter < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Failure to parse at {:?}" , self . 0 . line_col) } } impl < R > core :: error :: Error for MietteAdapter < R > where R : RuleType , Self : fmt :: Debug + fmt :: Display , { } }
};
}
