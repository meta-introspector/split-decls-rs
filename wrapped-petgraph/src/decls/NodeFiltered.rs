macro_rules! NodeFiltered {
    () => {
        # [doc = " A node-filtering graph adaptor."] # [derive (Copy , Clone , Debug)] pub struct NodeFiltered < G , F > (pub G , pub F) ;
    };
}

NodeFiltered!();