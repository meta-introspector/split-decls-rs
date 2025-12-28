macro_rules! deps {
    () => {
        Capabilities!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        # [cfg (windows)] impl Default for Capabilities { fn default () -> Self { Capabilities { precompose_unicode : false , ignore_case : true , executable_bit : false , symlink : false , } } }
    };
}

impl_1!()