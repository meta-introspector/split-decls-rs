macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { output_is_terminal : true , colored : true , timestamp : false , terminal_dimensions : (80 , 20) , hide_cursor : false , level_filter : None , initial_delay : None , frames_per_second : 6.0 , throughput : false , keep_running_if_progress_is_empty : true , } } }
    };
}

impl_82!()