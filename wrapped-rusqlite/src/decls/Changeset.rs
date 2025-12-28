macro_rules! Changeset {
    () => {
        # [doc = " Changeset or Patchset"] pub struct Changeset { cs : * mut c_void , n : c_int , }
    };
}

Changeset!()