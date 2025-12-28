macro_rules! write {
    () => {
        # [doc = ""] pub mod write { # [doc = " The error type returned by the [`Write`](crate::Write) trait."] pub type Error = Box < dyn std :: error :: Error + Send + Sync + 'static > ; }
    };
}

write!()