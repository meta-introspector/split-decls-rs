macro_rules! deps {
    () => {
        Capabilities!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [cfg (all (unix , not (target_os = "macos")))] impl Default for Capabilities { fn default () -> Self { Capabilities { precompose_unicode : false , ignore_case : false , executable_bit : true , symlink : true , } } }
    };
}

impl_3!();