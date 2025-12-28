macro_rules! deps {
    () => {
        Protocol!();
        Capabilities!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        # [doc = " This implementation yields exactly those minimal capabilities that are required for `gix` to work, nothing more and nothing less."] # [doc = ""] # [doc = " This is a bit of a hack just get tests with Protocol V0 to work, which is a good way to enforce stateful transports."] # [doc = " Of course, V1 would also do that but when calling `git-upload-pack` directly, it advertises so badly that this is easier to implement."] impl Default for Capabilities { fn default () -> Self { Capabilities :: from_lines ("version 2\nmulti_ack_detailed\nside-band-64k\n" . into ()) . expect ("valid format, known at compile time") } }
    };
}

impl_150!()