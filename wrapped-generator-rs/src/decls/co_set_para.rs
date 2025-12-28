macro_rules! deps {
    () => {
        ContextStack!();
    };
}

macro_rules! co_set_para {
    () => {
        deps!();
        # [doc = " set current coroutine para in user space"] pub fn co_set_para < A : Any > (para : A) { if let Some (ctx) = ContextStack :: current () . co_ctx () { ctx . co_set_para (para) } }
    };
}

co_set_para!()