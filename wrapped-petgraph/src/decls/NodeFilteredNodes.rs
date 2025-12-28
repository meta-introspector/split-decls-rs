macro_rules! NodeFilteredNodes {
    () => {
        # [doc = " A filtered node references iterator."] # [derive (Debug , Clone)] pub struct NodeFilteredNodes < 'a , I , F : 'a > { include_source : bool , iter : I , f : & 'a F , }
    };
}

NodeFilteredNodes!()