macro_rules! deps {
    () => {
        Capabilities!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        # [cfg (target_os = "macos")] impl Default for Capabilities { fn default () -> Self { Capabilities { precompose_unicode : true , ignore_case : true , executable_bit : true , symlink : true , } } }
    };
}

impl_2!()