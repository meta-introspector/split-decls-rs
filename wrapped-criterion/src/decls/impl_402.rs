macro_rules! deps {
    () => {
        Html!();
        WallTime!();
        Criterion!();
        Reports!();
        Measurement!();
        CliReport!();
        Duration!();
        BenchmarkFilter!();
        Baseline!();
        CliVerbosity!();
        SamplingMode!();
        BencherReport!();
        BenchmarkConfig!();
        Sample!();
        Mode!();
        ExternalProfiler!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl Default for Criterion { # [doc = " Creates a benchmark manager with the following default settings:"] # [doc = ""] # [doc = " - Sample size: 100 measurements"] # [doc = " - Warm-up time: 3 s"] # [doc = " - Measurement time: 5 s"] # [doc = " - Bootstrap size: 100 000 resamples"] # [doc = " - Noise threshold: 0.01 (1%)"] # [doc = " - Confidence level: 0.95"] # [doc = " - Significance level: 0.05"] # [doc = " - Plotting: enabled, using gnuplot if available or plotters if gnuplot is not available"] # [doc = " - No filter"] fn default () -> Criterion { let reports = Reports { cli_enabled : true , cli : CliReport :: new (false , false , CliVerbosity :: Normal) , bencher_enabled : false , bencher : BencherReport , html : default_plotting_backend () . create_plotter () . map (Html :: new) , csv_enabled : cfg ! (feature = "csv_output") , } ; let mut criterion = Criterion { config : BenchmarkConfig { confidence_level : 0.95 , measurement_time : Duration :: from_secs (5) , noise_threshold : 0.01 , nresamples : 100_000 , sample_size : 100 , significance_level : 0.05 , warm_up_time : Duration :: from_secs (3) , sampling_mode : SamplingMode :: Auto , quick_mode : false , } , filter : BenchmarkFilter :: AcceptAll , report : reports , baseline_directory : "base" . to_owned () , baseline : Baseline :: Save , load_baseline : None , output_directory : default_output_directory () . clone () , all_directories : HashSet :: new () , all_titles : HashSet :: new () , measurement : WallTime , profiler : Box :: new (RefCell :: new (ExternalProfiler)) , connection : cargo_criterion_connection () . as_ref () . map (| mtx | mtx . lock () . unwrap ()) , mode : Mode :: Benchmark , } ; if criterion . connection . is_some () { criterion . report . cli_enabled = false ; criterion . report . bencher_enabled = false ; criterion . report . csv_enabled = false ; criterion . report . html = None ; } criterion } }
    };
}

impl_402!();