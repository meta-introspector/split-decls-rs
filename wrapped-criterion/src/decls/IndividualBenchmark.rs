macro_rules! IndividualBenchmark {
    () => {
        # [derive (Serialize)] struct IndividualBenchmark { name : String , path : String , regression_exists : bool , }
    };
}

IndividualBenchmark!()