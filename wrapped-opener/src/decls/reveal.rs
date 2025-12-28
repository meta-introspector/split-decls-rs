macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! reveal {
    () => {
        deps!();
        # [doc = " Opens the default file explorer and reveals a file or folder in its containing folder."] # [doc = ""] # [doc = " ## Errors"] # [doc = " This function may or may not return an error if the path does not exist."] # [doc = ""] # [doc = " ## Platform Implementation Details"] # [doc = " - On Windows and Windows Subsystem for Linux (WSL) the `explorer.exe /select, <path>` command is used."] # [doc = " - On Mac the system `open -R` command is used."] # [doc = " - On non-WSL Linux the [`file-manager-interface`] or the [`org.freedesktop.portal.OpenURI`] DBus Interface is used if available,"] # [doc = "   falling back to opening the containing folder with [`open`]."] # [doc = " - On other platforms, the containing folder is shown with [`open`]."] # [doc = ""] # [doc = " [`org.freedesktop.portal.OpenURI`]: https://flatpak.github.io/xdg-desktop-portal/#gdbus-org.freedesktop.portal.OpenURI"] # [doc = " [`file-manager-interface`]: https://freedesktop.org/wiki/Specifications/file-manager-interface/"] # [cfg (feature = "reveal")] pub fn reveal < P > (path : P) -> Result < () , OpenError > where P : AsRef < std :: path :: Path > , { sys :: reveal (path . as_ref ()) }
    };
}

reveal!();