macro_rules! Limited {
    () => {
        # [doc = " I/O wrapper that limits the number of bytes written or read on each call."] # [doc = ""] # [doc = " See the [`limited`] and [`limited_write`] methods."] # [doc = ""] # [doc = " [`limited`]: super::AsyncReadTestExt::limited"] # [doc = " [`limited_write`]: super::AsyncWriteTestExt::limited_write"] # [pin_project] # [derive (Debug)] pub struct Limited < Io > { # [pin] io : Io , limit : usize , }
    };
}

Limited!()