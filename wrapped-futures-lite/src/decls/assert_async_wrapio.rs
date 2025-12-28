macro_rules! assert_async_wrapio {
    () => {
        fn assert_async_wrapio < F , T > (mut f : F) -> Poll < std :: io :: Result < T > > where F : FnMut () -> std :: io :: Result < T > , { loop { match f () { Err (err) if err . kind () == ErrorKind :: Interrupted => { } res => return Poll :: Ready (res) , } } }
    };
}

assert_async_wrapio!()