macro_rules! Reference {
    () => {
        pub enum Reference < 'b , 'c , T > where T : ? Sized + 'static , { Borrowed (& 'b T) , Copied (& 'c T) , }
    };
}

Reference!();