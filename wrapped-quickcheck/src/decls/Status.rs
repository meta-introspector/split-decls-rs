macro_rules! Status {
    () => {
        # [doc = " Whether a test has passed, failed or been discarded."] # [derive (Clone , Debug , PartialEq)] enum Status { Pass , Fail , Discard , }
    };
}

Status!()