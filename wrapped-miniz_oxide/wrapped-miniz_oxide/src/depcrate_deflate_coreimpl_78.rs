// Generated macro for impl_78 (impl)
macro_rules! Depcrate_deflate_coreimpl_78 {
() => {
// Module: crate::deflate::core
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a > CallbackOxide < 'a > { fn new_callback_buf (in_buf : & 'a [u8] , out_buf : & 'a mut [u8]) -> Self { CallbackOxide { in_buf : Some (in_buf) , in_buf_size : None , out_buf_size : None , out : CallbackOut :: Buf (CallbackBuf { out_buf }) , } } fn new_callback_func (in_buf : & 'a [u8] , callback_func : CallbackFunc < 'a >) -> Self { CallbackOxide { in_buf : Some (in_buf) , in_buf_size : None , out_buf_size : None , out : CallbackOut :: Func (callback_func) , } } fn update_size (& mut self , in_size : Option < usize > , out_size : Option < usize >) { if let (Some (in_size) , Some (size)) = (in_size , self . in_buf_size . as_mut ()) { * * size = in_size ; } if let (Some (out_size) , Some (size)) = (out_size , self . out_buf_size . as_mut ()) { * * size = out_size } } fn flush_output (& mut self , saved_output : SavedOutputBufferOxide , params : & mut ParamsOxide ,) -> i32 { if saved_output . pos == 0 { return params . flush_remaining as i32 ; } self . update_size (Some (params . src_pos) , None) ; match self . out { CallbackOut :: Func (ref mut cf) => cf . flush_output (saved_output , params) , CallbackOut :: Buf (ref mut cb) => cb . flush_output (saved_output , params) , } } pub (crate) fn buf (& mut self) -> Option < & 'a [u8] > { self . in_buf } }
};
}
