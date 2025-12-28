macro_rules! deps {
    () => {
        Throughput!();
    };
}

macro_rules! BenchmarkId {
    () => {
        deps!();
        # [derive (Clone , Serialize , Deserialize , PartialEq , Eq)] pub struct BenchmarkId { pub group_id : String , pub function_id : Option < String > , pub value_str : Option < String > , pub throughput : Option < Throughput > , full_id : String , directory_name : String , title : String , }
    };
}

BenchmarkId!();