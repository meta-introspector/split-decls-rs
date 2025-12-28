macro_rules! deps {
    () => {
        RegionCtxt!();
    };
}

macro_rules! ToArgRegionsFolder {
    () => {
        deps!();
        # [doc = " A folder to map the regions in the hidden type to their corresponding `arg_regions`."] # [doc = ""] # [doc = " This folder has to differentiate between member regions and other regions in the hidden"] # [doc = " type. Member regions have to be equal to one of the `arg_regions` while other regions simply"] # [doc = " get treated as an existential region in the opaque if they are not. Existential"] # [doc = " regions are currently represented using `'erased`."] struct ToArgRegionsFolder < 'a , 'tcx > { rcx : & 'a RegionCtxt < 'a , 'tcx > , erase_unknown_regions : bool , arg_regions : & 'a [RegionVid] , }
    };
}

ToArgRegionsFolder!()