macro_rules! deps {
    () => {
        ContextStack!();
    };
}

macro_rules! get_local_data {
    () => {
        deps!();
        # [doc = " get the current context local data"] # [doc = " only coroutine support local data"] # [inline] pub fn get_local_data () -> * mut u8 { let env = ContextStack :: current () ; env . co_ctx () . map_or (ptr :: null_mut () , | ctx | ctx . local_data) }
    };
}

get_local_data!();