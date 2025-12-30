// Generated macro for impl_20 (impl)
macro_rules! Depcrate_fragmentsimpl_20 {
() => {
// Module: crate::fragments
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > sval :: Stream < 'a > for TextCollector < 'a > { fn text_begin (& mut self , _ : Option < usize >) -> sval :: Result { Ok (()) } fn text_fragment (& mut self , fragment : & 'a str) -> sval :: Result { self . try_catch (| buf | buf . push_fragment (fragment)) } fn text_fragment_computed (& mut self , fragment : & str) -> sval :: Result { self . try_catch (| buf | buf . push_fragment_computed (fragment)) } fn text_end (& mut self) -> sval :: Result { Ok (()) } fn null (& mut self) -> sval :: Result { self . fail (Error :: unsupported ("text" , "null")) } fn bool (& mut self , _ : bool) -> sval :: Result { self . fail (Error :: unsupported ("text" , "boolean")) } fn i64 (& mut self , _ : i64) -> sval :: Result { self . fail (Error :: unsupported ("text" , "integer")) } fn f64 (& mut self , _ : f64) -> sval :: Result { self . fail (Error :: unsupported ("text" , "floating point")) } fn seq_begin (& mut self , _ : Option < usize >) -> sval :: Result { self . fail (Error :: unsupported ("text" , "sequence")) } fn seq_value_begin (& mut self) -> sval :: Result { self . fail (Error :: unsupported ("text" , "sequence")) } fn seq_value_end (& mut self) -> sval :: Result { self . fail (Error :: unsupported ("text" , "sequence")) } fn seq_end (& mut self) -> sval :: Result { self . fail (Error :: unsupported ("text" , "sequence")) } }
};
}
