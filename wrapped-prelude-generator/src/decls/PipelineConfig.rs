macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! PipelineConfig {
    () => {
        deps!();
        pub type PipelineConfig = pipeline_traits :: Config ;
    };
}

PipelineConfig!();