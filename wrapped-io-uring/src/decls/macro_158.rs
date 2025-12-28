macro_rules! macro_158 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (io_uring_use_own_sys)] { include ! (env ! ("IO_URING_OWN_SYS_BINDING")) ; } else if # [cfg (all (feature = "bindgen" , not (feature = "overwrite")))] { include ! (concat ! (env ! ("OUT_DIR") , "/sys.rs")) ; } else { include ! ("sys.rs") ; } }
    };
}

macro_158!()