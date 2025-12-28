macro_rules! CsvRow {
    () => {
        # [derive (Serialize)] struct CsvRow < 'a > { group : & 'a str , function : Option < & 'a str > , value : Option < & 'a str > , throughput_num : Option < & 'a str > , throughput_type : Option < & 'a str > , sample_measured_value : f64 , unit : & 'static str , iteration_count : u64 , }
    };
}

CsvRow!()