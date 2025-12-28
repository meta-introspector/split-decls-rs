macro_rules! deps {
    () => {
        Platform!();
        ResourceRef!();
        DriverChoice!();
        Options!();
    };
}

macro_rules! PlatformRef {
    () => {
        deps!();
        # [doc = " The product of a [`prepare_merge()`](Platform::prepare_merge()) call to finally"] # [doc = " perform the merge and retrieve the merge results."] # [derive (Copy , Clone)] pub struct PlatformRef < 'parent > { # [doc = " The platform that hosts the resources, used to access drivers."] pub (super) parent : & 'parent Platform , # [doc = " The current or our side of the merge operation."] pub current : ResourceRef < 'parent > , # [doc = " The ancestor or base of the merge operation."] pub ancestor : ResourceRef < 'parent > , # [doc = " The other or their side of the merge operation."] pub other : ResourceRef < 'parent > , # [doc = " Which driver to use according to the resource's configuration,"] # [doc = " using the path of `current` to read git-attributes."] pub driver : DriverChoice , # [doc = " Possibly processed options for use when performing the actual merge."] # [doc = ""] # [doc = " They may be inspected before the merge, or altered at will."] pub options : platform :: merge :: Options , }
    };
}

PlatformRef!();