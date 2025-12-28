macro_rules! deps {
    () => {
        SmartSubtransport!();
    };
}

macro_rules! RawSmartSubtransport {
    () => {
        deps!();
        # [doc = " Instance of a `git_smart_subtransport`, must use `#[repr(C)]` to ensure that"] # [doc = " the C fields come first."] # [repr (C)] struct RawSmartSubtransport { raw : raw :: git_smart_subtransport , stream : Option < * mut raw :: git_smart_subtransport_stream > , rpc : bool , obj : Box < dyn SmartSubtransport > , }
    };
}

RawSmartSubtransport!()