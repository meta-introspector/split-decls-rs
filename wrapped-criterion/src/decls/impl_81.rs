macro_rules! deps {
    () => {
        Throughput!();
        Result!();
        BenchmarkId!();
        MeasurementData!();
        CsvRow!();
        CsvReportWriter!();
        ValueFormatter!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < W : Write > CsvReportWriter < W > { fn write_data (& mut self , id : & BenchmarkId , data : & MeasurementData < '_ > , formatter : & dyn ValueFormatter ,) -> Result < () > { let mut data_scaled : Vec < f64 > = data . sample_times () . as_ref () . into () ; let unit = formatter . scale_for_machines (& mut data_scaled) ; let group = id . group_id . as_str () ; let function = id . function_id . as_deref () ; let value = id . value_str . as_deref () ; let (throughput_num , throughput_type) = match id . throughput { Some (Throughput :: Bytes (bytes)) => (Some (format ! ("{}" , bytes)) , Some ("bytes")) , Some (Throughput :: BytesDecimal (bytes)) => (Some (format ! ("{}" , bytes)) , Some ("bytes")) , Some (Throughput :: Elements (elems)) => (Some (format ! ("{}" , elems)) , Some ("elements")) , Some (Throughput :: Bits (bits)) => (Some (format ! ("{}" , bits)) , Some ("bits")) , None => (None , None) , } ; let throughput_num = throughput_num . as_deref () ; for (count , measured_value) in data . iter_counts () . iter () . zip (data_scaled) { let row = CsvRow { group , function , value , throughput_num , throughput_type , sample_measured_value : measured_value , unit , iteration_count : (* count) as u64 , } ; self . writer . serialize (row) ? ; } Ok (()) } }
    };
}

impl_81!()