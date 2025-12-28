macro_rules! deps {
    () => {
        RawBenchmarkId!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl From < & InternalBenchmarkId > for RawBenchmarkId { fn from (other : & InternalBenchmarkId) -> RawBenchmarkId { RawBenchmarkId { group_id : other . group_id . clone () , function_id : other . function_id . clone () , value_str : other . value_str . clone () , throughput : other . throughput . iter () . cloned () . collect () , } } }
    };
}

impl_67!()