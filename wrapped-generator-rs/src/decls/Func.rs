macro_rules! Func {
    () => {
        pub struct Func { data : * mut () , size : usize , offset : * mut usize , func : fn (* mut ()) , drop : fn (* mut ()) , }
    };
}

Func!();