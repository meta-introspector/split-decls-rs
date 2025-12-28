macro_rules! Service {
    () => {
        # [doc = " Actions that a smart transport can ask a subtransport to perform"] # [derive (Copy , Clone , PartialEq , Debug)] # [allow (missing_docs)] pub enum Service { UploadPackLs , UploadPack , ReceivePackLs , ReceivePack , }
    };
}

Service!()