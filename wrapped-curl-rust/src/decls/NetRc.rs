macro_rules! NetRc {
    () => {
        # [doc = " Options for `.netrc` parsing."] # [derive (Debug , Clone , Copy)] pub enum NetRc { # [doc = " Ignoring `.netrc` file and use information from url"] # [doc = ""] # [doc = " This option is default"] Ignored = curl_sys :: CURL_NETRC_IGNORED as isize , # [doc = " The  use of your `~/.netrc` file is optional, and information in the URL is to be"] # [doc = " preferred. The file will be scanned for the host and user name (to find the password only)"] # [doc = " or for the host only, to find the first user name and password after that machine, which"] # [doc = " ever information is not specified in the URL."] Optional = curl_sys :: CURL_NETRC_OPTIONAL as isize , # [doc = " This value tells the library that use of the file is required, to ignore the information in"] # [doc = " the URL, and to search the file for the host only."] Required = curl_sys :: CURL_NETRC_REQUIRED as isize , }
    };
}

NetRc!()