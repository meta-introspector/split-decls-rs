macro_rules! AnyValueId {
    () => {
        # [derive (Copy , Clone)] pub struct AnyValueId { type_id : std :: any :: TypeId , # [cfg (debug_assertions)] type_name : & 'static str , }
    };
}

AnyValueId!();