macro_rules! deps {
    () => {
        DurationFormatter!();
        Throughput!();
        ValueFormatter!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl ValueFormatter for DurationFormatter { fn scale_throughputs (& self , typical : f64 , throughput : & Throughput , values : & mut [f64] ,) -> & 'static str { match * throughput { Throughput :: Bytes (bytes) => self . bytes_per_second (bytes as f64 , typical , values) , Throughput :: BytesDecimal (bytes) => { self . bytes_per_second_decimal (bytes as f64 , typical , values) } Throughput :: Elements (elems) => self . elements_per_second (elems as f64 , typical , values) , Throughput :: Bits (bits) => self . bits_per_second (bits as f64 , typical , values) , } } fn scale_values (& self , ns : f64 , values : & mut [f64]) -> & 'static str { let (factor , unit) = if ns < 10f64 . powi (0) { (10f64 . powi (3) , "ps") } else if ns < 10f64 . powi (3) { (10f64 . powi (0) , "ns") } else if ns < 10f64 . powi (6) { (10f64 . powi (- 3) , "µs") } else if ns < 10f64 . powi (9) { (10f64 . powi (- 6) , "ms") } else { (10f64 . powi (- 9) , "s") } ; for val in values { * val *= factor ; } unit } fn scale_for_machines (& self , _values : & mut [f64]) -> & 'static str { "ns" } }
    };
}

impl_161!()