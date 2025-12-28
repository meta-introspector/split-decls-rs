macro_rules! deps {
    () => {
        Options!();
        Progress!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { title : "Progress Dashboard" . into () , frames_per_second : 10.0 , throughput : false , recompute_column_width_every_nth_frame : None , window_size : None , stop_if_progress_missing : true , } } }
    };
}

impl_54!();