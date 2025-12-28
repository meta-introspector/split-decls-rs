macro_rules! PushUpdate {
    () => {
        # [doc = " Represents an update which will be performed on the remote during push."] pub struct PushUpdate < 'a > { raw : * const raw :: git_push_update , _marker : marker :: PhantomData < & 'a raw :: git_push_update > , }
    };
}

PushUpdate!();