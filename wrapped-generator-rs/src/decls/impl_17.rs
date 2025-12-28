macro_rules! deps {
    () => {
        Generator!();
        GeneratorObj!();
        GeneratorImpl!();
        StackBox!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a , A , T , const LOCAL : bool > GeneratorObj < 'a , A , T , LOCAL > { # [doc = " Constructs a Generator from a raw pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe because improper use may lead to"] # [doc = " memory problems. For example, a double-free may occur if the"] # [doc = " function is called twice on the same raw pointer."] # [inline] pub unsafe fn from_raw (raw : * mut usize) -> Self { GeneratorObj { gen : StackBox :: from_raw (raw as * mut GeneratorImpl < 'a , A , T >) , } } # [doc = " Consumes the `Generator`, returning a wrapped raw pointer."] # [inline] pub fn into_raw (self) -> * mut usize { let ret = self . gen . as_ptr () as * mut usize ; std :: mem :: forget (self) ; ret } # [doc = " prefetch the generator into cache"] # [inline] pub fn prefetch (& self) { self . gen . prefetch () ; } # [doc = " prepare the para that passed into generator before send"] # [inline] pub fn set_para (& mut self , para : A) { self . gen . set_para (para) ; } # [doc = " set the generator local data"] # [inline] pub fn set_local_data (& mut self , data : * mut u8) { self . gen . set_local_data (data) ; } # [doc = " get the generator local data"] # [inline] pub fn get_local_data (& self) -> * mut u8 { self . gen . get_local_data () } # [doc = " get the generator panic data"] # [inline] pub fn get_panic_data (& mut self) -> Option < Box < dyn Any + Send > > { self . gen . get_panic_data () } # [doc = " resume the generator without touch the para"] # [doc = " you should call `set_para` before this method"] # [inline] pub fn resume (& mut self) -> Option < T > { self . gen . resume () } # [doc = " `raw_send`"] # [inline] pub fn raw_send (& mut self , para : Option < A >) -> Option < T > { self . gen . raw_send (para) } # [doc = " send interface"] pub fn send (& mut self , para : A) -> T { self . gen . send (para) } # [doc = " cancel the generator"] # [doc = " this will trigger a Cancel panic to unwind the stack and finish the generator"] pub fn cancel (& mut self) { self . gen . cancel () } # [doc = " is finished"] # [inline] pub fn is_done (& self) -> bool { self . gen . is_done () } # [doc = " get stack total size and used size in word"] pub fn stack_usage (& self) -> (usize , usize) { self . gen . stack_usage () } }
    };
}

impl_17!()