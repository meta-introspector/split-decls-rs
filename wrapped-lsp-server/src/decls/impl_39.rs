macro_rules! deps {
    () => {
        RequestId!();
        Request!();
        Outgoing!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < O > Outgoing < O > { pub fn register < P : serde :: Serialize > (& mut self , method : String , params : P , data : O) -> Request { let id = RequestId :: from (self . next_id) ; self . pending . insert (id . clone () , data) ; self . next_id += 1 ; Request :: new (id , method , params) } pub fn complete (& mut self , id : RequestId) -> Option < O > { self . pending . remove (& id) } }
    };
}

impl_39!();