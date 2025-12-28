macro_rules! hyper_context {
    () => {
        # [doc = " An async context for a task that contains the related waker."] # [doc = ""] # [doc = " This is provided to `hyper_io`'s read and write callbacks. Currently"] # [doc = " its only purpose is to provide access to the waker. See `hyper_waker`."] # [doc = ""] # [doc = " Corresponding Rust type: <https://doc.rust-lang.org/std/task/struct.Context.html>"] pub struct hyper_context < 'a > (Context < 'a >) ;
    };
}

hyper_context!();