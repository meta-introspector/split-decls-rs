macro_rules! deps {
    () => {
        Throughput!();
    };
}

macro_rules! RawBenchmarkId {
    () => {
        deps!();
        # [derive (Debug , Serialize)] pub struct RawBenchmarkId { group_id : String , function_id : Option < String > , value_str : Option < String > , throughput : Vec < Throughput > , }
    };
}

RawBenchmarkId!()