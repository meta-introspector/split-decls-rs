macro_rules! hyper_buf {
    () => {
        # [doc = " A buffer of bytes that is sent or received on a `hyper_body`."] # [doc = ""] # [doc = " Obtain one of these in the callback of `hyper_body_foreach` or by receiving"] # [doc = " a task of type `HYPER_TASK_BUF` from `hyper_executor_poll` (after calling"] # [doc = " `hyper_body_data` and pushing the resulting task)."] # [doc = ""] # [doc = " Methods:"] # [doc = ""] # [doc = " - hyper_buf_bytes: Get a pointer to the bytes in this buffer."] # [doc = " - hyper_buf_copy:  Create a new hyper_buf * by copying the provided bytes."] # [doc = " - hyper_buf_free:  Free this buffer."] # [doc = " - hyper_buf_len:   Get the length of the bytes this buffer contains."] pub struct hyper_buf (pub (crate) Bytes) ;
    };
}

hyper_buf!();