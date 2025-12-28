macro_rules! deps {
    () => {
        OdbPackwriterCb!();
    };
}

macro_rules! OdbPackwriter {
    () => {
        deps!();
        # [doc = " A stream to write a packfile to the ODB"] pub struct OdbPackwriter < 'repo > { raw : * mut raw :: git_odb_writepack , progress : raw :: git_indexer_progress , progress_payload_ptr : * mut OdbPackwriterCb < 'repo > , }
    };
}

OdbPackwriter!();