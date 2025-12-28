macro_rules! deps {
    () => {
        RegContext!();
        Context!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl Context { # [doc = " return a default generator context"] pub fn new () -> Context { Context { regs : RegContext :: empty () , para : MaybeUninit :: zeroed () , ret : MaybeUninit :: zeroed () , _ref : 1 , err : None , child : ptr :: null_mut () , parent : ptr :: null_mut () , local_data : ptr :: null_mut () , stack_guard : (0 , 0) , } } # [doc = " judge it's generator context"] # [inline] pub fn is_generator (& self) -> bool { ! std :: ptr :: eq (self . parent , self) } # [doc = " get current generator send para"] # [inline] pub fn get_para < A > (& mut self) -> Option < A > where A : Any , { let para = unsafe { let para_ptr = * self . para . as_mut_ptr () ; assert ! (! para_ptr . is_null ()) ; & mut * para_ptr } ; match para . downcast_mut :: < Option < A > > () { Some (v) => v . take () , None => type_error :: < A > ("get yield type mismatch error detected") , } } # [doc = " get coroutine send para"] # [inline] pub fn co_get_para < A > (& mut self) -> Option < A > { let para = unsafe { let para_ptr = * self . para . as_mut_ptr () ; debug_assert ! (! para_ptr . is_null ()) ; & mut * (para_ptr as * mut Option < A >) } ; para . take () } # [doc = " set coroutine send para"] # [doc = " without check the data type for coroutine performance reason"] # [inline] pub fn co_set_para < A > (& mut self , data : A) { let para = unsafe { let para_ptr = * self . para . as_mut_ptr () ; debug_assert ! (! para_ptr . is_null ()) ; & mut * (para_ptr as * mut Option < A >) } ; * para = Some (data) ; } # [doc = " set current generator return value"] # [inline] pub fn set_ret < T > (& mut self , v : T) where T : Any , { let ret = unsafe { let ret_ptr = * self . ret . as_mut_ptr () ; assert ! (! ret_ptr . is_null ()) ; & mut * ret_ptr } ; match ret . downcast_mut :: < Option < T > > () { Some (r) => * r = Some (v) , None => type_error :: < T > ("yield type mismatch error detected") , } } # [doc = " set coroutine return value"] # [doc = " without check the data type for coroutine performance reason"] # [inline] pub fn co_set_ret < T > (& mut self , v : T) { let ret = unsafe { let ret_ptr = * self . ret . as_mut_ptr () ; debug_assert ! (! ret_ptr . is_null ()) ; & mut * (ret_ptr as * mut Option < T >) } ; * ret = Some (v) ; } }
    };
}

impl_35!();